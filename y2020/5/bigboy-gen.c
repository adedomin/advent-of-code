#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define MIN 85204
#define MAX 16862420
#define LINES 1024*1024*16
#define LENGTH 25

#define FILENAME "bigboy"

int main(void){
	time_t t;
	srand((unsigned) time(&t));

	int *random = calloc(LINES, sizeof(int));
	for(int i = 0; i < LINES; ++i){
		random[i] = i + MIN;
	}
	for (int i = 0; i < LINES - 1; i++){
		int j = i + rand() / (RAND_MAX / (LINES - i) + 1);
		int t = random[j];
		random[j] = random[i];
		random[i] = t;
	}
	printf("missing: %i\n", random[LINES - 1]);
	FILE *output = fopen(FILENAME, "w");
	char buffer [64];
	for(int i = 0; i < LINES - 1; ++i){
		for(int j = 0; j < LENGTH - 7; ++j){
			if(random[i] & (1 << (LENGTH - 1 - j)))
				buffer[j] = 'B';
			else
				buffer[j] = 'F';
		}
		for(int j = LENGTH - 7; j < LENGTH; ++j){
			if(random[i] & (1 << (LENGTH - 1 - j)))
				buffer[j] = 'R';
			else
				buffer[j] = 'L';
		}
		buffer[LENGTH] = '\n';
		buffer[LENGTH + 1] = '\0';
		fputs(buffer, output);
	}
	fclose(output);
}
