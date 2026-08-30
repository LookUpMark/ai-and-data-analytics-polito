.data
    v1: .double 0.0, 23.87, 0.0, 65.12, 92.34, 0.0, 12.45, 0.0
        .double 78.90, 44.76, 0.0, 38.29, 91.67, 0.0, 55.33, 0.0
        .double 64.92, 0.0, 23.47, 0.0, 98.12, 50.65, 0.0, 87.12
        .double 34.78, 0.0, 29.35, 0.0, 76.24, 0.0, 41.88, 53.29

    v2: .double 12.56, 0.0, 45.78, 0.0, 29.34, 0.0, 78.90, 31.45
        .double 0.0, 67.89, 12.56, 0.0, 54.12, 88.90, 0.0, 24.78
        .double 0.0, 37.65, 0.0, 48.91, 73.34, 0.0, 16.87, 0.0
        .double 65.34, 42.67, 0.0, 19.23, 0.0, 87.98, 31.02, 0.0

    v3: .double 23.56, 44.89, 78.12, 91.34, 12.65, 67.89, 88.23, 55.67
        .double 34.12, 97.23, 45.78, 65.43, 81.92, 29.34, 63.21, 90.78
        .double 54.19, 72.34, 37.89, 18.67, 92.56, 61.45, 83.12, 40.78
        .double 99.01, 76.23, 52.89, 33.45, 48.67, 86.12, 24.65, 69.78
        
    v4: .space 256
    v5: .space 256  
    v6: .space 256
    a: .space 8
    b: .space 8

.code
    MAIN:   daddui    R1, R0, 248     # R1 = 248 (initial index for arrays)
            daddi     R2, R0, -8      # R2 = -8 (decrement for index)
            daddui    R3, R0, 3       # R3 = 3 (used for division)
            daddi     R7, R0, 1       # R7 = 1 (initialization)

    LOOP1:  ddiv      R4, R1, R3      # R4 = R1 / R3
            l.d       F1, v1(R1)      # Load v1[R1] into F1
            dmul      R5, R4, R3      # R5 = R4 * R3
            l.d       F9, b(R0)       # Load b into F9
            l.d       F3, v2(R1)      # Load v2[R1] into F3
 
    ELSE1:  dmul      R8, R7, R1      # R8 = R7 * R1 (multiply)
            mtc1      R8, F8          # Move R8 to F8 (integer to floating-point)
            mul.d     F2, F1, F8      # F2 = F1 * F8 (multiply)
            mfc1      R7, F2          # Move F2 back to R7 (floating-point to integer)

    LOOP11: mul.d     F5, F1, F2      # F5 = F1 * F2
            l.d       F4, v3(R1)      # Load v3[R1] into F4
            sub.d     F6, F5, F3      # F6 = F5 - F3
            div.d     F7, F6, F4      # F7 = F6 / F4
            sub.d     F10, F6, F1     # F10 = F6 - F1
            sub.d     F8, F7, F9      # F8 = F7 - F9 (F8 = final result of expression)
            s.d       F6, v4(R1)      # Store F6 into v4[R1]
            mul.d     F11, F10, F8    # F11 = F10 * F8
            s.d       F8, v5(R1)      # Store F8 into v5[R1]
            dadd      R1, R1, R2      # Decrement R1 by 8 (move to next element)
            s.d       F11, v6(R1)     # Store F11 into v6[R1]

    LOOP2:  ddiv      R4, R1, R3      # R4 = R1 / R3
            l.d       F1, v1(R1)      # Load v1[R1] into F1
            dmul      R5, R4, R3      # R5 = R4 * R3
            l.d       F9, b(R0)       # Load b into F9
            l.d       F3, v2(R1)      # Load v2[R1] into F3

    IF2:    dsllv     R8, R7, R1      # R8 = R7 << R1 (shift left R7 by R1 bits)
            mtc1      R8, F8          # Move R8 to F8 (integer to floating-point)
            div.d     F2, F1, F8      # F2 = F1 / F8 (divide)
            mfc1      R7, F2          # Move F2 back to R7 (floating-point to integer)

    LOOP22: mul.d     F5, F1, F2      # F5 = F1 * F2
            l.d       F4, v3(R1)      # Load v3[R1] into F4
            sub.d     F6, F5, F3      # F6 = F5 - F3
            div.d     F7, F6, F4      # F7 = F6 / F4
            sub.d     F10, F6, F1     # F10 = F6 - F1
            sub.d     F8, F7, F9      # F8 = F7 - F9 (F8 = final result of expression)
            s.d       F6, v4(R1)      # Store F6 into v4[R1]
            mul.d     F11, F10, F8    # F11 = F10 * F8
            s.d       F8, v5(R1)      # Store F8 into v5[R1]
            dadd      R1, R1, R2      # Decrement R1 by 8 (move to next element)
            beq R1, R2, END
            s.d       F11, v6(R1)     # Store F11 into v6[R1]

    LOOP3:  ddiv      R4, R1, R3      # R4 = R1 / R3
            l.d       F1, v1(R1)      # Load v1[R1] into F1
            dmul      R5, R4, R3      # R5 = R4 * R3
            l.d       F9, b(R0)       # Load b into F9
            l.d       F3, v2(R1)      # Load v2[R1] into F3

    ELSE3:  dmul      R8, R7, R1      # R8 = R7 * R1 (multiply)
            mtc1      R8, F8          # Move R8 to F8 (integer to floating-point)
            mul.d     F2, F1, F8      # F2 = F1 * F8 (multiply)
            mfc1      R7, F2          # Move F2 back to R7 (floating-point to integer)

    LOOP33: mul.d     F5, F1, F2      # F5 = F1 * F2
            l.d       F4, v3(R1)      # Load v3[R1] into F4
            sub.d     F6, F5, F3      # F6 = F5 - F3
            div.d     F7, F6, F4      # F7 = F6 / F4
            sub.d     F10, F6, F1     # F10 = F6 - F1
            sub.d     F8, F7, F9      # F8 = F7 - F9 (F8 = final result of expression)
            s.d       F6, v4(R1)      # Store F6 into v4[R1]
            mul.d     F11, F10, F8    # F11 = F10 * F8
            s.d       F8, v5(R1)      # Store F8 into v5[R1]
            dadd      R1, R1, R2      # Decrement R1 by 8 (move to next element)
            j LOOP1
            s.d       F11, v6(R1)     # Store F11 into v6[R1]
            
    END:    HALT                      # End of program