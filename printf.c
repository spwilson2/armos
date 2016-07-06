#define MAX_INT_CHARS   (32/4)
#define NIBBLE_MASK     0x0000000f
#define DECIMAL_MAX     0xffffffff

// Return 0 on success -1 on failure.
int printf(const char *fmt, ...) {
    void *args = &fmt;
    int i;

    // TODO: FIXME: Once we implement kmalloc, malloc this instead of storing
    // on stack.
    char decimal_string[MAX_INT_CHARS + 1];

    // TODO: Parse for the args based off of format characters.
    for (i=0; i < strlen(fmt); i++) {
        //TODO: Handle the escape char.
        if (fmt[i] == '%') {
            switch(fmt[i+1]) {
                case 's':
                    break;
                case 'x':
                    break;
                case 'u':
                    break;
                case 'd':
                    break;
                default:
                    //TODO FAIL.
                    return -1;

            }

        }
        
    }
    return 0;
}

void int_to_decimal (int val, char * buf) {
    int remainder = val;
    while (remainder) {

    //TODO: Check if it's negative or positive.
    
}

void int_to_hex(int val, char * buf) {
    short nibble;
    char  next_char;

    // Null terminate the hex string.
    buf[MAX_INT_CHARS] = 0;

    for (nibble = MAX_INT_CHARS - 1; nibble >= 0 ; nibble--) {
        next_char = (char)((val >> (nibble*4)) & NIBBLE_MASK);

        // Store the ascii value into the hex string.
        if (next_char <= 9)
            buf[MAX_INT_CHARS - 1 - nibble] = next_char + '0';
        else
            buf[MAX_INT_CHARS - 1 - nibble] = next_char + 'A';
    }
}
