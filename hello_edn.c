#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "hardware/gpio.h"
#include "pico/stdlib.h"

void some_edn(char* edn);

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    char buf[40];
    int count = 0;
    while (true) {
        sleep_ms(500);
        gpio_put(LED_PIN, 1);
        sprintf(buf, "{:count %d :foo #{1 2 3 2 42}}", count);
        some_edn(buf);
        count++;
        sleep_ms(500);
        gpio_put(LED_PIN, 0);
    }
}
