# Data section
.data
v1:    .double  0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7
       .double  8.8, 9.9, 10.10, 11.11, 12.12, 13.13, 14.14, 15.15
       .double  16.16, 17.17, 18.18, 19.19, 20.20, 21.21, 22.22, 23.23
       .double  24.24, 25.25, 26.26, 27.27, 28.28, 29.29, 30.30, 31.31
       # Array of 32 double values

v2:    .double  0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5
       .double  8.5, 9.5, 10.50, 11.51, 12.52, 13.53, 14.54, 15.55
       .double  16.56, 17.57, 18.58, 19.59, 20.60, 21.61, 22.62, 23.63
       .double  24.64, 25.65, 26.66, 27.67, 28.68, 29.69, 30.70, 31.71
       # Array of 32 double values

v3:    .double  1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0
       .double  9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0
       .double  17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0
       .double  25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0
       # Array of 32 double values

v4:    .space 256    # Space for 32 double values (32 * 8 bytes = 256 bytes)
v5:    .space 256    # Space for 32 double values
v6:    .space 256    # Space for 32 double values

a:    .double 13.2    # Single double value
b:    .double 18.5    # Single double value

# Code section
.code
MAIN:   daddui    R1, R0, 248     # R1 = 248 (initial index for arrays)
        daddi     R2, R0, -8      # R2 = -8 (decrement for index)
        daddui    R3, R0, 3       # R3 = 3 (used for division)
        daddi     R7, R0, 1       # R7 = 1 (m)

LOOP1:  l.d       F1, v1(R1)      # Load v1[R1] into F1
        l.d       F9, b(R0)       # Load b into F9
        ddiv      R4, R1, R3      # R4 = R1 / R3
        dmul      R5, R4, R3      # R5 = R4 * R3
        bne       R1, R5, ELSE    # If R1 is not equal to R5, go to ELSE

IF:     dsllv     R8, R7, R1      # R8 = R7 << R1 (shift left R7 by R1 bits)
        mtc1      R8, F8          # Move R8 to F8 (integer to floating-point)
        div.d     F2, F1, F8      # F2 = F1 / F8 (divide)
        mfc1      R7, F2          # Move F2 back to R7 (floating-point to integer)
        j         LOOP2           # Jump to LOOP2

ELSE:   dmul      R8, R7, R1      # R8 = R7 * R1 (multiply)
        mtc1      R8, F8          # Move R8 to F8 (integer to floating-point)
        mul.d     F2, F1, F8      # F2 = F1 * F8 (multiply)
        mfc1      R7, F2          # Move F2 back to R7 (floating-point to integer)

LOOP2:  l.d       F3, v2(R1)      # Load v2[R1] into F3
        l.d       F4, v3(R1)      # Load v3[R1] into F4
        mul.d     F5, F1, F2      # F5 = F1 * F2
        sub.d     F6, F5, F3      # F6 = F5 - F3
        div.d     F7, F6, F4      # F7 = F6 / F4
        sub.d     F8, F7, F9      # F8 = F7 - F9 (F8 = final result of expression)
        sub.d     F10, F6, F1     # F10 = F6 - F1
        mul.d     F11, F10, F8    # F11 = F10 * F8
        s.d       F6, v4(R1)      # Store F6 into v4[R1]
        s.d       F8, v5(R1)      # Store F8 into v5[R1]
        s.d       F11, v6(R1)     # Store F11 into v6[R1]
        dadd      R1, R1, R2      # Decrement R1 by 8 (move to next element)
        bne       R1, R2, LOOP1   # If R1 is not equal to -8, repeat the loop

        HALT                      # End of program
