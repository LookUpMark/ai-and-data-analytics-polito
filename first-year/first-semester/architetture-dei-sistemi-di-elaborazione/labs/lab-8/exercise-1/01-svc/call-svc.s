			AREA svc_code, CODE, READONLY
			EXPORT call_svc

call_svc 	STMFD SP!, {R0-R12, LR}
			SVC 0x6
			