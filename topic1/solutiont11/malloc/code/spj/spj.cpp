#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
using namespace std;

int valid_ans;
float u_ans, t_ans;

const char *flag_base ="77796755c34b3aada95b7ab2caf33d5ec337d567e58a328cf02c139b29bc650c";

int main(int argc, char **argv) {
    FILE *in = fopen(argv[1], "r");    //输入文件名
    FILE *out = fopen(argv[2], "r");   //选手输出文件名
    FILE *ans = fopen(argv[3], "r");   //标准输出文件名
    FILE *full = fopen(argv[4], "r");  //满分文件名
    FILE *scr = fopen(argv[5], "w");   //分数文件名，spj自己创建
    FILE *log = fopen(argv[6], "w");   //额外信息文件名,spj自己创建

    /**
     * malloc这题将最终正确性信息以及两个指标输出到记录分数文件内
     *
     * 格式如下：
     * iscorrect
     * u
     * t
     *
     *
     */

    char flag[256]={0};

    int ret = fscanf(out, "%100s %d %f %f", flag, &valid_ans, &u_ans, &t_ans);

    if (ret != 4) {
        fprintf(log, "Output error!\n");
        fprintf(scr, "0\n1.000000 999999.0000000");
        goto exit;
    }

    if (strcmp(flag, flag_base)) {
        fprintf(log, "Flag not match!\n");
        fprintf(scr, "0\n1.000000 999999.0000000\n");
        goto exit;
    }
    fprintf(scr, "%d\n", valid_ans);
    fprintf(scr, "u: %.6f t(s): %.6f\n", 1-u_ans, t_ans);
    // fprintf(scr, "%.6f\n", t_ans);

    if (valid_ans == 1) {
        fprintf(log, "pass correctness test\n");
    } else if (valid_ans == 0) {
        fprintf(log, "fail correctness test\n");
    } else {
        fprintf(
            log,
            "valid_and is not 1 or 0, maybe something is wrong in output.\n");
    }

exit:
    fclose(in);
    fclose(out);
    fclose(ans);
    fclose(full);
    fclose(scr);
    fclose(log);
}
