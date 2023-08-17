use std::future::Future;
use std::os::raw::{c_int, c_void};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time;

// Record 学生信息，id、height
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    id: c_int,
    height: c_int,
}

/*
           ******************************* 第一版 *******************************
           1. 自己实现future，使用atomic 通知
*/

// 函数参数，第一个参数实际上对应的是Record 第二个参数是个closure。第一个参数实际上是closure执行时的参数。
pub type GetRecord<T> = unsafe extern "C" fn(*mut T, *mut c_void);

extern "C" {
    pub fn query(id: c_int, fnptr: GetRecord<Record>, closure: *mut c_void);
}

// hook 与GetRecord函数签名一致。我们期望c能够传递一个record 到我们的closure中，我们处理完之后，c再free record
unsafe extern "C" fn hook<F, T>(record: *mut T, closure: *mut c_void)
where
    F: FnOnce(*mut T),
{
    let closure = Box::from_raw(closure as *mut F); // from_raw，使得closure 可以释放内存
    closure(record);
}

// get_callback 返回一个函数指针
pub fn get_callback<F, T>(_closure: &F) -> GetRecord<T>
where
    F: FnOnce(*mut T),
{
    hook::<F, T>
}

struct QueryFuture {
    query_id: c_int, // 用于传给c接口，模拟作为查询参数
    state: AtomicBool, // 用于通知Future是否结束
    result: Option<Record>, // 用于返回查询结果
}

impl Future for QueryFuture {
    type Output = Record;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Record> {
        if self.state.load(Ordering::Relaxed) {
            Poll::Ready(self.result.take().unwrap())
        } else {
            let waker = cx.waker().clone();
            let q_id = self.query_id;
            unsafe {
                let closure = Box::new(move |r: *mut Record| {
                    self.result = Some(Record {
                        id: (*r).id,
                        height: (*r).height,
                    });
                    self.state.store(true, Ordering::Relaxed);
                    println!("next to wake...");
                    waker.wake(); // 继续poll task
                    sleep(time::Duration::from_secs(2));
                    println!("wake success");
                });
                let closure_ptr = Box::leak(closure); // 这里leak的box，会在hock() 里free
                let callback = get_callback(closure_ptr);
                query(q_id, callback, closure_ptr as *mut _ as *mut c_void);
            }
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let future = QueryFuture {
        query_id: 3,
        state: AtomicBool::new(false),
        result: None,
    };

    let out = future.await;
    println!("get record is :{:#?} {:#?}", out.id, out.height);
    sleep(time::Duration::from_secs(3));
}
/*
queryfn start
query fn end
thfn start
next to wake...
get record is :5123 34
wake success
thfn done
*/