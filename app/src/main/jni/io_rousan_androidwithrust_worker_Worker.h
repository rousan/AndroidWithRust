/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class io_rousan_androidwithrust_worker_Worker */

#ifndef _Included_io_rousan_androidwithrust_worker_Worker
#define _Included_io_rousan_androidwithrust_worker_Worker
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     io_rousan_androidwithrust_worker_Worker
 * Method:    startRustWorker
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_io_rousan_androidwithrust_worker_Worker_startRustWorker
  (JNIEnv *, jobject);

/*
 * Class:     io_rousan_androidwithrust_worker_Worker
 * Method:    sendMessageToWorker
 * Signature: (Ljava/lang/String;Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_io_rousan_androidwithrust_worker_Worker_sendMessageToWorker
  (JNIEnv *, jobject, jstring, jstring);

#ifdef __cplusplus
}
#endif
#endif
