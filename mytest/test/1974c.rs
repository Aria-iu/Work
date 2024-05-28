fn main() {}

/*
ans in C++ 20
*/
#include<bits/stdc++.h>
using namespace std;
typedef long long ll;

int i,j,k,n,m,t,a[1005000];
ll res;

int main(){
    ios::sync_with_stdio(0); cin.tie(0);
    cin>>t;
    while(t--){
        map<tuple<int,int,int>,int> mp1;
        map<pair<int,int> ,int> mp[4];
        cin>>n;
        for(i=1;i<=n;i++)cin>>a[i];
        res=0;
        for(i=3;i<=n;i++){
            res+=mp[1][{a[i-1],a[i]}]++;
            res+=mp[2][{a[i-2],a[i]}]++;
            res+=mp[3][{a[i-1],a[i-2]}]++;
            res-=mp1[{a[i-2],a[i-1],a[i]}]*3;
            mp1[{a[i-2],a[i-1],a[i]}]++;
        }
        cout<<res<<'\n';
    }
}