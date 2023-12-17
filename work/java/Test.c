#include <stdio.h>
#include "Test.h"
JNIEXPORT void JNICALL Java_Test_echo(JNIEnv *env, jclass cl, jint a){
	printf("hello echo %d\n",a);
}
