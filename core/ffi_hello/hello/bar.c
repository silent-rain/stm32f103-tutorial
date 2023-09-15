int AD_Value[4];

/* 函数返回两个数中较大的那个数 */
int max(int num1, int num2) 
{
   /* 局部变量声明 */
   int result;
 
   if (num1 > num2) {
      result = num1;
   } else {
      result = num2;
   }
   return result; 
}