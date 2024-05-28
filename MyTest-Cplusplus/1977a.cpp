//
// Created by zyc on 24-5-26.
//

#include <iostream>
#define FOR(num) for (;num>0;num--)
#define FOR2(i,len) for (int i = 0; i< len;i++)

using namespace std;

void
solve(int len,int h){
    if (h>len){
        printf("No\n");
        return;
    }else{
        if ((h-len)%2==0){
            printf("Yes\n");
        }else{
            printf("No\n");
        }
    }
}

int
main(){
    int num;
    cin>>num;
    FOR(num){
        int nums;
        int h;
        cin >> nums;
        cin >> h;
        solve(nums,h);
    }

    return 0;
}
