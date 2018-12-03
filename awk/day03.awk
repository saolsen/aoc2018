#!/usr/bin/env awk -f
{   
    match($0, /#[0-9]+ @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)/, arr)
    x[NR]      = arr[1]
    y[NR]      = arr[2]
    width[NR]  = arr[3]
    height[NR] = arr[4]
}
END {
    for (i = 1; i <= NR; i = i + 1) {
        #printf("{id: %i, x: %i, y: %i, width: %i height: %i}\n", i, x[i], y[i], width[i], height[i])
        base = y[i] * 1000 + x[i];
        for (h = 0; h < height[i]; h = h + 1) {
            for (w = 0; w < width[i]; w = w + 1) {
                fabric[base+h*1000+w] += 1;
            }
        }
    }

    num_overlapped = 0;
    for (i = 0; i < 1000*1000; i++) {
        if (fabric[i] > 1) {
            num_overlapped += 1;
        }
    }

    printf("Num Overlapped Inches: %i\n", num_overlapped);

    for (i = 1; i <= NR; i = i + 1) {
        base = y[i] * 1000 + x[i];
        found = 1;
        for (h = 0; h < height[i]; h = h + 1) {
            for (w = 0; w < width[i]; w = w + 1) {
                if (fabric[base+h*1000+w] != 1) {
                    found = 0;
                }
            }
        }
        if (found) {
            printf("Only Non Overlapping Claim: %i\n", i);
            break;
        }
    }
}
