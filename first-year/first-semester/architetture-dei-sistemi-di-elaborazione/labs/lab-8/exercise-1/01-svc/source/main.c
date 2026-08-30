__attribute__((naked)) void call_svc(void)
{
	__asm(
	
			"AREA svc_code, CODE, READONLY"
			"EXPORT svc_code"
"svc_code	SVC 0x10"
			
	
	)
	
	
};

int main (void){

	call_svc();

}
