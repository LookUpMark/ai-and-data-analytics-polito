.data
    v1:     .byte 2, 6, -3, 11, 9, 18, -13, 16, 5, 1  # Vector v1
    v2:     .byte 4, 2, -13, 3, 9, 9, 7, 16, 4, 7     # Vector v2
    v3:     .byte 0,0,0,0,0,0,0,0,0,0                 # Vector v3 (output)
    flag1:  .byte 1                                   # Flag 1 (initially 1)
    flag2:  .byte 0                                   # Flag 2 (initially 0)
    flag3:  .byte 0                                   # Flag 3 (initially 0)

.code
    MAIN:   
            daddui  R1, R0, 10        # R1 = 10 (dimension of vectors)
            daddui  R2, R0, 0         # R2 = 0 (v1 index counter)
            daddui  R3, R0, 0         # R3 = 0 (v2 index counter)
            daddui  R4, R0, 0         # R4 = 0 (v3 index counter)
            daddui  R5, R0, 0         # R5 = 0 (store v1 current element)
            daddui  R6, R0, 0         # R6 = 0 (store v2 current element)
            daddui  R7, R0, 1         # R7 = 1 (store flag1)
            daddui  R8, R0, 0         # R8 = 0 (store flag2)
            daddui  R9, R0, 0         # R9 = 0 (store flag3)
            daddui  R10, R0, 0        # R10 = 0 (flag1 counter)

    LOOP:   
            beq     R2, R1, DECREASE  # If R2 == 10 (end of v1), jump to DECREASE
            lb      R5, v1(R2)        # Load byte from v1 at index R2 into R5
            lb      R6, v2(R3)        # Load byte from v2 at index R3 into R6
            beq     R5, R6, EQUAL     # If v1[R2] == v2[R3], jump to EQUAL
            daddui  R2, R2, 1         # Increment v1 index (R2)
            j       LOOP              # Repeat the loop

    EQUAL:  
            beq     R10, R0, FULL     # If R10 == 0, jump to FULL (v3 is empty)
            sb      R5, v3(R4)        # Store R5 (v1 element) in v3 at index R4
            daddui  R2, R2, 1         # Increment v1 index (R2)
            daddui  R4, R4, 1         # Increment v3 index (R4)
            j       LOOP              # Continue looping

    FULL:   
            daddui  R7, R0, 0         # Set flag1 to 0
            sb      R7, flag1(R0)     # Store flag1 (0) in memory
            daddui  R10, R10, 1       # Increment flag1 counter (R10)
            j       EQUAL             # Go back to EQUAL logic

    DECREASE: 
            dadd    R2, R0, R0        # Reset v1 index counter (R2 = 0)
            daddui  R3, R3, 1         # Increment v2 index counter (R3)
            beq     R3, R1, RESET1    # If R3 == 10, jump to RESET1
            j       LOOP              # Otherwise, go back to LOOP

    RESET1: 
            daddui  R3, R0, 1         # Reset v2 index counter to 1 (R3 = 1)
            j       CHECK1            # Jump to CHECK1 to start comparison

    CHECK1: 
            lb      R5, v3(R2)        # Load v3[i] into R5
            lb      R6, v3(R3)        # Load v3[i+1] into R6
            slt     R8, R5, R6        # Set R8 = 1 if v3[i] < v3[i+1]
            beqz    R8, RESET2        # If R8 == 0 (v3[i] >= v3[i+1]), jump to RESET2
            daddui  R2, R2, 1         # Increment i counter (R2)
            daddui  R3, R3, 1         # Increment i+1 counter (R3)
            beq     R3, R4, RESET2    # If i+1 == max size of v3, jump to RESET2
            j       CHECK1            # Otherwise, continue comparing

    RESET2: 
            daddui  R2, R0, 0         # Reset i counter (R2 = 0)
            daddui  R3, R0, 1         # Reset i+1 counter (R3 = 1)
            sb      R8, flag2(R0)     # Store flag2 (result of last comparison)
            j       CHECK2            # Jump to CHECK2 for the next comparison

    CHECK2: 
            lb      R5, v3(R2)        # Load v3[i] into R5
            lb      R6, v3(R3)        # Load v3[i+1] into R6
            slt     R9, R6, R5        # Set R9 = 1 if v3[i+1] < v3[i]
            beqz    R9, END           # If R9 == 0 (v3[i+1] >= v3[i]), jump to END
            daddui  R2, R2, 1         # Increment i counter (R2)
            daddui  R3, R3, 1         # Increment i+1 counter (R3)
            beq     R3, R4, END       # If i+1 == max size of v3, jump to END
            j       CHECK2            # Otherwise, continue checking

    END:    
            sb      R9, flag3(R0)     # Store flag3 (result of the last comparison)
            HALT                      # End the program
