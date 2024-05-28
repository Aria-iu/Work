//
// Created by zyc on 24-5-23.
//
#include <iostream>
#define FOR(num) for (;num>0;num--)
#define FOR2(i,len) for (int i = 0; i< len;i++)

using namespace std;

void
solve(int len,string str){
    int x = 0, y = 0;
    FOR2(i,len){
        if (str[i]=='N'){
            y++;
        }else if (str[i]=='E'){
            x++;
        }else if (str[i]=='W'){
            x--;
        }else{
            y--;
        }
    }
    if ((x%2!=0)||(y%2!=0)||(x==0&&y==0&&len==2)){
        printf("NO\n");
        return;
    }
    string T(len,'R');
    if (x==0&&y==0){
        T[0] = 'H';
        for (int i =1 ;i< len;i++){
            if ((str[0]=='N'&&str[i]=='S')||(str[0]=='S'&&str[i]=='N')||(str[0]=='E'&&str[i]=='W')||(str[0]=='W'&&str[i]=='E'))
            {
                T[i] = 'H';
                break;
            }
        }
        cout<< T <<'\n';
        return;
    }
    // 找到这个点
    x/=2;
    y/=2;
    for(int i = 0;i<len;i++){
        if (str[i]=='N'&&y>0) {T[i]='H';y--;}
        else if (str[i]=='S'&&y<0) {T[i]='H';y++;}
        else if (str[i]=='E'&&x>0) {T[i]='H';x--;}
        else if (str[i]=='W'&&x<0) {T[i]='H';x++;}

        if (x==0&&y==0) break;
    }
    cout<< T <<'\n';
    return;
}

int
main(){
    int num;
    cin>>num;
    FOR(num){
        int len;
        string str;
        cin >> len;
        cin >> str;
        solve(len,str);
    }

    return 0;
}