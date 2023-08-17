// example03/src/dbmanager.c
#include <stdio.h>
#include <pthread.h>

typedef struct Record
{
  int id;
  int height;
} Record;
typedef void (*WakeCallbackExecutor)(void *closure);

typedef struct TArgs2
{
  int *res_int;
  Record *record;
  void *callback_ptr;
  WakeCallbackExecutor callback_executor;
} TArgs2;

void thfn2(void *arg)
{
  sleep(2);
  printf("thfn2 start\n");
  TArgs2 *targs = (TArgs2 *)arg;
  *targs->res_int = 4;
  targs->record->id = 11233;
  targs->record->height = 99999;
  targs->callback_executor(targs->callback_ptr);
  printf("thfn2 done\n");
}

void query2(int id, int *res_int, Record *res, WakeCallbackExecutor callback_executor, void *callback_ptr)
{
  printf("query2 start\n");
  pthread_t thread1;
  TArgs2 *targs = (TArgs2 *)(malloc(sizeof(TArgs2)));
  targs->callback_ptr = callback_ptr;
  targs->callback_executor = callback_executor;
  targs->res_int = res_int;
  targs->record = res;
  pthread_create(&thread1, NULL, (void *)&thfn2, (void *)targs);
  pthread_detach(thread1);
  printf("query2 fn end\n");
}