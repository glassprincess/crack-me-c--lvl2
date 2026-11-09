using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.ExceptionServices;
using System.Runtime.InteropServices;
using System.Runtime.Intrinsics;
using System.Text;

namespace Soba4kiBrugautPoStenam
{
    internal class Program
    {
        static void Main(string[] args)
        {
            byte[] res = new byte[] { 61, 23, 26, 28, 0, 90, 90, 90, 14, 41, 62, 23, 23, 2, 44, 74, 21, 57, 41, 75, 56, 75, 53, 60, 41, 58, 47, 40, 90, 90, 90, 6 };
            const byte resKey = 0x7B;

            int a1 = 943, a2 = 163, a3 = 34;
            int useriq = 120;
            string b1 = "Yts", b2 = "rEw", b3 = "uJkl";
            string RTT = $"{(a1 ^ a2) / a3}" + b1 + $"{a3 / a2}" + b2 + b3;
            string password = "improreverser000"; // true password
            string fakepassword = "Flag{ImPr0ReverseP}"; // fake password, do not fall for it !
            Console.WriteLine("Crackme Password LVL - 2");
            Console.Write("Enter password : ");
            string userpass = Console.ReadLine();

            // some scam logics ))

            if (userpass == b1)
            {
                useriq -= 20;
            }
            if (userpass == b2)
            {
                useriq -= 15;
            }
            if (userpass == b3)
            {
                useriq -= 35;
            }

            // another scam

            if (useriq * useriq < 0)
            {
                Console.WriteLine("Correct Password! You win !");
                useriq = 0;
                Thread.Sleep(1000);
                goto sobaka;
            }

            // real prog logics


            if (userpass == RTT)
            {
                Console.WriteLine(" ");
                Console.WriteLine("Correct Password! You win !");
                goto realwin;
            }
            else if (userpass != RTT)
            {
                Console.WriteLine(" ");
                Console.WriteLine("!!! WRONG !!!");
            }  
            else if (userpass == password)
            {
                Console.WriteLine(" ");
                Console.WriteLine("u r scammed bro, try again lol");
            }

            // scam moment lol4ik izi vasya

        Console.WriteLine(" ");
        Console.Write("Press any key to leave : ");
        Console.ReadKey();


        return;

        sobaka:

            Console.WriteLine("bro i just scammed u lol");
            Console.WriteLine("WRONGGGGGG!!!!!!!!!");

        realwin:

            byte[] res2 = new byte[res.Length];
            for (int i = 0; i < res.Length; i++)
            {
                res[i] = (byte)(res[i] ^ resKey);
            }
            Console.WriteLine($"Your Flag is - " + Encoding.UTF8.GetString(res));
        }
    }
}