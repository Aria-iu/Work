//
// Created by zyc on 24-5-2.
//
// gcc error_function.c ./sys_test/user_group/check_passwd.c -lcrypt -o check_passwd
// 使用管理员权限执行
/*
 * `-lcrypt` 是一个编译选项，它告诉编译器链接 `crypt` 库。在这种情况下，`crypt` 库提供了加密和解密密码的函数。
 * 在你的程序中，你使用了 `crypt` 函数来加密用户输入的密码。
 * 但是，这个函数实际上是定义在 `crypt` 库中的。因此，你需要在编译时指定 `-lcrypt` 选项，
 * 以便编译器能够找到并链接 `crypt` 库中的函数实现。
 *
 * */
#include <pwd.h>
#include <shadow.h>
#include <unistd.h>
#include <stdbool.h>
#include "../total.h"


int
main(int argc, char *argv[])
{
    char *username, *password, *encrypted, *p;
    struct passwd *pwd;
    struct spwd *spwd;
    bool authOk;
    size_t len;
    long lnmax;

    lnmax = sysconf(_SC_LOGIN_NAME_MAX);
    if (lnmax == -1)                    /* If limit is indeterminate */
        lnmax = 256;                    /* make a guess */

    username = malloc(lnmax);
    if (username == NULL)
        errExit("malloc");

    printf("Username: ");
    fflush(stdout);
    if (fgets(username, lnmax, stdin) == NULL)
        exit(EXIT_FAILURE);             /* Exit on EOF */

    len = strlen(username);
    if (username[len - 1] == '\n')
        username[len - 1] = '\0';       /* Remove trailing '\n' */

    pwd = getpwnam(username);
    if (pwd == NULL)
        fatal("couldn't get password record");
    spwd = getspnam(username);
    if (spwd == NULL && errno == EACCES)
        fatal("no permission to read shadow password file");

    if (spwd != NULL)           /* If there is a shadow password record */
        pwd->pw_passwd = spwd->sp_pwdp;     /* Use the shadow password */

    password = getpass("Password: ");

    /* Encrypt password and erase cleartext version immediately */

    encrypted = crypt(password, pwd->pw_passwd);
    for (p = password; *p != '\0'; )
        *p++ = '\0';

    if (encrypted == NULL)
        errExit("crypt");

    authOk = strcmp(encrypted, pwd->pw_passwd) == 0;
    if (!authOk) {
        printf("Incorrect password\n");
        exit(EXIT_FAILURE);
    }

    printf("Successfully authenticated: UID=%ld\n", (long) pwd->pw_uid);

    /* Now do authenticated work... */

    exit(EXIT_SUCCESS);
}