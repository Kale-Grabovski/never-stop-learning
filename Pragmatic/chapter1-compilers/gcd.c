
#include <stdio.h>

int getint() {
	int n;
	scanf("%d", &n);
	return n;
}

void putint(int n) {
	printf("%d", n);
}

int main() {
	int i = getint(), j = getint();
	while (i != j) {
		if (i > j) {
			i = i - j;
		} else {
			j = j - i;
		}
	}

	putint(i);
}

