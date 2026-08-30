.data
v1:    .double  0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7
       .double  8.8, 9.9, 10.10, 11.11, 12.12, 13.13, 14.14, 15.15
       .double  16.16, 17.17, 18.18, 19.19, 20.20, 21.21, 22.22, 23.23
       .double  24.24, 25.25, 26.26, 27.27, 28.28, 29.29, 30.30, 31.31

v2:    .double  0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5
       .double  8.5, 9.5, 10.50, 11.51, 12.52, 13.53, 14.54, 15.55
       .double  16.56, 17.57, 18.58, 19.59, 20.60, 21.61, 22.62, 23.63
       .double  24.64, 25.65, 26.66, 27.67, 28.68, 29.69, 30.70, 31.71

v3:    .double  1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0
       .double  9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0
       .double  17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0
       .double  25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0

v4:    .space 256    # 32 double-precision values (32 * 8 bytes = 256 bytes)
v5:    .space 256    # 32 double-precision values
v6:    .space 256    # 32 double-precision values

.code
MAIN:
       daddi  R1, R0, 31    # Starting value of the loop (R1 = 31)
       daddi  R2, R0, -1    # Loop decrement value (R2 = -1)

LOOP:
       beq    R1, R2, END   # Exit loop if R1 == R2

       dsll   R3, R1, 3     # R3 = R1 * 8 (shift left by 3 to get byte offset for double)

       l.d    F1, v1(R3)    # Load v1[R1] into F1
       l.d    F2, v2(R3)    # Load v2[R1] into F2
       mul.d  F3, F1, F1    # F3 = F1 * F1
       sub.d  F4, F3, F2    # F4 = F3 - F2
       s.d    F4, v4(R3)    # Store F4 into v4[R1]

       l.d    F3, v3(R3)    # Load v3[R1] into F3
       div.d  F6, F4, F3    # F6 = F4 / F3
       sub.d  F5, F6, F2    # F5 = F6 - F2
       s.d    F5, v5(R3)    # Store F5 into v5[R1]

       sub.d  F7, F4, F1    # F7 = F4 - F1
       mul.d  F6, F7, F5    # F6 = F7 * F5
       s.d    F6, v6(R3)    # Store F6 into v6[R1]

       daddi  R1, R1, -1    # Decrement R1 by 1
       j      LOOP          # Jump back to LOOP
        
END:
       halt                 # End the program