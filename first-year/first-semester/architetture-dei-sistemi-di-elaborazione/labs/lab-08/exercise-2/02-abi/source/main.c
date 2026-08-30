#include<stdio.h>
#include<stdint.h>

extern int Matrix_Coordinates[];
extern uint8_t ROWS;
extern uint8_t COLUMNS;

extern int Check_Square(int, int, int);
extern float My_division(float*, float*);

int main(void){

	volatile int result = 0;
	volatile float div = 0;
	int x = 0;
	int y = 0;
	int r = 5;
	
	int i;
	for(i=0; i < ROWS*COLUMNS; i=i+2){
		
		x = Matrix_Coordinates[i];
		y = Matrix_Coordinates[i+1];
		
		result = result + Check_Square(x, y, r);
	}; 
	
	float A = (float)result;
	float R = (float)r*r;
	div = My_division(&A, &R);
	
	while(1);
}
