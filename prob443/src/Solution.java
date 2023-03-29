class Solution {
    public int compress(char[] chars) {

        int wi = 0;
        for(int i = 0; i < chars.length;) {
            char c = chars[i];
            int j = 1;
            for(; j + i < chars.length; j++)
                if (chars[i+j] != c)
                    break;
            i += j;
            chars[wi++] = c;

            if (j == 1) continue;
            int md = 1000;
            while(md > 0) {
                if (j > md) {
                    chars[wi++] = (char) ('0' + ((char) (j / md)));
                    j = j % md;
                } else if (j == md) {
                    chars[wi++] = '1';
                    while(md > 1) {
                        chars[wi++] = '0';
                        md /= 10;
                    }
                }
                md = md / 10;
            }
        }
        return wi;
    }
}