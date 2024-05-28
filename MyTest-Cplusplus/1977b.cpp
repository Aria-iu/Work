//
// Created by zyc on 24-5-26.
//


#include <iostream>
#define FOR(num) for (;num>0;num--)
#define FOR2(i,len) for (int i = 0; i< len;i++)

using namespace std;

void resolve(int n,int bits[32]){
    int temp=2;
    int idx = 1;
    while(n>=temp){
        temp*=2;
        idx++;
    }
    temp/=2;
    idx--;
    while (temp>1&&n!=0){
        if (n>=temp){
            n-=temp;
            temp/=2;
            // display[idx] = 1;
            bits[idx] = 1;
            idx--;
        }else{
            temp/=2;
            bits[idx] = 0;
            idx--;
        }


    }
}



void
solve(int n){
    // unsigned int display = 0;
    int len = 0;
    char display[32] = {0};
    int bit_s[32] = {0};

    if (n&1){
        // odd
        n--;
        bit_s[0] = 1;
        resolve(n,bit_s);

    }else{
        // even
        resolve(n,bit_s);
    }

    for (int i = 0;i<32;i++){
        if (bit_s[i]==0){
            continue;
        }
        int temp_idx = i;
        int temp_idx_end = 0;

        for (int j = i;j<32;j++){
            if (bit_s[j]==0){
                temp_idx_end = j;
                break;
            }
        }
        if (temp_idx_end-temp_idx==1){
            continue;
        }
        bit_s[temp_idx] = -1;
        for (int t = temp_idx+1;t<temp_idx_end;t++){
            bit_s[t] = 0;
        }
        bit_s[temp_idx_end] = 1;
    }

    int length = 0;
    for(int index = 31;index>=0;index--){
        if (bit_s[index]!=0){
            length = index+1;
            break;
        }
    }
    printf("%d\n",length);
    for(int i = 0;i<length;i++){
        printf("%d ",bit_s[i]);
    }
    printf("");
}

int
main(){
    int num;
    cin>>num;
    FOR(num){
        int nums;
        cin >> nums;
        solve(nums);
    }
    return 0;
}
