				AREA asm_functions, CODE, READONLY			
                EXPORT  Check_Square
				EXPORT My_division
				IMPORT __aeabi_fdiv
				
				
Check_Square	STMFD SP!, {r4-r7,lr}
				MUL R0, R0, R0
				MUL R1, R1, R1
				MUL R2, R2, R2
				ADD R3, R0, R1
				CMP R3, R2
				MOVLE R0, #1
				MOVGT R0, #0 
				LDMFD SP!, {r4-r7, pc}
				
My_division		STMFD SP!, {r4-r7,lr}
				LDR R0, [R0]
				LDR R1, [R1]
				BL __aeabi_fdiv
				LDMFD SP!, {r4-r7, pc}
							
				END
				