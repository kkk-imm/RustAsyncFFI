// example03/src/main.rs
use std::os::raw::{c_int, c_void};
use std::sync::Arc;
use std::thread::sleep;
use std::time;

use tokio::sync::Notify;
pub type WakeCallbackExecutor = unsafe extern "C" fn(*mut c_void);

// Record 学生信息，id、height
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    id: c_int,
    height: c_int,
}

extern "C" {
    pub fn query2(
        id: c_int,
        res_int: *mut c_int,
        res: *mut Record,
        callback_executor: WakeCallbackExecutor,
        callback_ptr: *mut c_void,
    );
}

unsafe extern "C" fn hook2<F>(closure: *mut c_void)
where
    F: FnOnce(),
{
    let closure = Box::from_raw(closure as *mut F);
    closure();
}

pub fn get_callback2<F>(_closure: &F) -> WakeCallbackExecutor
where
    F: FnOnce(),
{
    hook2::<F>
}

pub async fn async_query2(id: c_int, res_int: SendPtr<*mut c_int>, res: SendPtr<*mut Record>) {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    let closure = Box::new(move || {
        notify2.notify_one(); // c侧的callback，唤醒async_query2
    });
    let closure_ptr = Box::leak(closure);
    let callback = get_callback2(closure_ptr);
    unsafe {
        query2(
            id,
            res_int.0,
            res.0,
            callback,
            closure_ptr as *mut _ as *mut c_void,
        );
    }
    notify.notified().await;
    println!("notify success");
}

#[derive(Debug)]
#[repr(transparent)]
pub struct SendPtr<T>(T);
unsafe impl<T> Send for SendPtr<T> {}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let mut record = Record::default();
        let mut res_int: c_int = 0;
        let send_ptr_res_int = SendPtr(&mut res_int as *mut c_int);
        let send_ptr_record = SendPtr(&mut record as *mut Record);
        async_query2(4, send_ptr_res_int, send_ptr_record).await;
        println!("res_int {:#?}", res_int); // 要保证 res_id 和 record 跨await，不然会有segment fault 的风险
        println!("record {:#?}", record);
    });
    sleep(time::Duration::from_secs(5));
}
