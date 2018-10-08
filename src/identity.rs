// 用户与身份
/*

用户系统即管理所有用户的系统。每个用户(user)定义为代表一个真实的填表人。

同一个用户可能有多个验证方式(approaches to authenticate)，如手机短信登录、QQ登录、微信登录、
账号密码登录等。每个验证方式只对应一个用户，如一个手机号只能和一位用户绑定。

产生一个新用户的方法称作注册(register)，注册应至少使用一种验证方式。

通过不同途径注册可能会产生多个用户，可以通过合并(merge)方法将这些用户合并。如通过手机短信注册了用户A，
用QQ注册了用户B，就能合并A和B而产生用户C，同时删除原来的用户A和B。合并用户注意，如果两个用户的验证方式有冲突，
就不能使用合并操作，如两个用户分别绑定不同的手机号时就不能合并。

//暂时头脑风暴到这，稍后写rfc

 */
