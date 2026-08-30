AREA function, CODE, READONLY
EXPORT next_state

next_state
    LDR R0, [R0] // current_state / new_state
	LDR R1, [R1] // taps
	LDR R2, [R2] // output_bit address
	MOV R3, #0	 // input_bit
	
	AND R2, R0, #1
	STR R2, [R2]
	
	EOR R3, R0, R0, LSR #2
	EOR R3, R3, R0, LSR #3
	EOR R3, R3, R0, LSR #4
	AND R3, R3, #1
	
	MOV R0, R0, LSR #1
	ORR R0, R0, R3, LSL #7