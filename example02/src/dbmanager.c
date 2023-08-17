// gcc -fPIC -shared -o libdbmanager.so dbmanager.c
#include <stdio.h>
#include <pthread.h>

typedef struct Record
{
  int id;
  int height;
} Record;

typedef void (*GetRecord)(Record *rec, void *closure);

// TArgs thread args
typedef struct TArgs 
{
  void *closure;
  GetRecord fnptr;
} Targs;

void thfn(void *arg)
{
  sleep(3);
  printf("thfn start\n");
  Targs *targs = (Targs *)arg;
  Record *r = (Record *)malloc(sizeof(Record));
  r->height = 34;
  r->id = 5123;
  targs->fnptr(r, targs->closure);
  // rust侧使用拷贝的数据。record使用完之后，c侧free自己的内存，而不是交给rust去free。
  // 因为二者用的内存分配器不是同一个，会有坑。
  free(r);
  free(targs);
  printf("thfn done\n");
}

void query(int id, GetRecord fnptr, void *closure)
{
  printf("queryfn start\n");
  pthread_t thread1;
  Targs *targs = (Targs *)(malloc(sizeof(Targs)));
  targs->closure = closure;
  targs->fnptr = fnptr;

  // c侧就直接用pthread去创建多线程操作
  pthread_create(&thread1, NULL, (void *)&thfn, (void *)targs);
  pthread_detach(thread1);
  printf("query fn end\n");
}
