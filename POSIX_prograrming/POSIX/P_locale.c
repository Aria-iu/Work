#include <stdio.h>
#include <locale.h>
#include <string.h>
#include <stdlib.h>

int P_locale() {
    char *locale;
    // 设置 LC_COLLATE 类别为 en_US.UTF-8 语言环境
    if ((locale = setlocale(LC_COLLATE, "POSIX")) == NULL) {
        perror("setlocale");
        return 1;
    }
    printf("当前 LC_COLLATE 语言环境为：%s\n", locale);



    // 获取并打印当前的 LC_COLLATE 语言环境
    locale= setlocale(LC_COLLATE, NULL);
    if (locale == NULL) {
        perror("setlocale");
        return 1;
    }
    printf("当前 LC_COLLATE 语言环境为：%s\n", locale);

    return 0;
}

void P_Mon_Locale(){
    // 设置LC_MONETARY类别为本地环境
    setlocale(LC_MONETARY, "en_US.UTF-8");

    // 获取货币格式化信息
    struct lconv *lc = localeconv();

    // 输出货币符号和小数点
    printf("货币符号: %s\n", lc->currency_symbol);
    printf("小数点: %s\n", lc->mon_decimal_point);

    char *locale = setlocale(LC_COLLATE, NULL);
    if (locale == NULL) {
        perror("setlocale");
    }
}


// 定义货币信息结构体
struct MonetaryInfo {
    char *currency_symbol;
    char *mon_decimal_point;
};

// 函数原型
void parse_monetary_info(const char *locale_definition, struct MonetaryInfo *info);

int P_analyse_Locale() {
    // 定义示例 locale 定义字符串
    const char *locale_definition = "LC_MONETARY\n"
                                    "currency_symbol \"$\"\n"
                                    "mon_decimal_point \".\"\n"
                                    "END LC_MONETARY\n";

    // 创建货币信息结构体
    struct MonetaryInfo info;

    // 解析并输出货币信息
    parse_monetary_info(locale_definition, &info);
    printf("货币符号: %s\n", info.currency_symbol);
    printf("小数点: %s\n", info.mon_decimal_point);

    return 0;
}

// 解析货币信息
void parse_monetary_info(const char *locale_definition, struct MonetaryInfo *info) {
    // 使用 strtok 函数分割字符串
    char buffer[1024]; // 适当调整缓冲区大小
    strcpy(buffer, locale_definition);
    // 使用 strtok 分割字符串
    char *token = strtok(buffer, "\n");
    
    // 初始化货币信息结构体
    info->currency_symbol = NULL;
    info->mon_decimal_point = NULL;

    // 解析每行内容
    while (token != NULL) {
        // 使用 sscanf 函数提取关键字和值
        char keyword[50], value[50];
        sscanf(token, "%s %s", keyword, value);

        // 根据关键字设置货币信息结构体的相应字段
        if (strcmp(keyword, "currency_symbol") == 0) {
            // 使用 strdup 函数复制字符串，并检查返回的指针是否为空
            info->currency_symbol = strdup(value);
            if (info->currency_symbol == NULL) {
                fprintf(stderr, "Error: Failed to allocate memory for currency symbol\n");
                exit(EXIT_FAILURE);
            }
        } else if (strcmp(keyword, "mon_decimal_point") == 0) {
            // 使用 strdup 函数复制字符串，并检查返回的指针是否为空
            info->mon_decimal_point = strdup(value);
            if (info->mon_decimal_point == NULL) {
                fprintf(stderr, "Error: Failed to allocate memory for monetary decimal point\n");
                exit(EXIT_FAILURE);
            }
        }

        // 继续读取下一行
        token = strtok(NULL, "\n");
    }
}