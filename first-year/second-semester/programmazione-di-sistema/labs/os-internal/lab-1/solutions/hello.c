#include <types.h>
#include <lib.h>

#include "hello.h"

void hello(char *msg) {
	if (msg!=NULL){
		kprintf("%s\n",msg);
	}
	else{
		kprintf("Hello OS/161\n");
	}
	
}