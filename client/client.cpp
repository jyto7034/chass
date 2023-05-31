#ifndef CLIENT_MAIN_H
#define CLIENT_MAIN_H

#include <WinSock2.h>

#include <iostream>
using std::cout;
using std::endl;

#include <thread>
#include <chrono>

#include <string>
#include <stdexcept>

#define SERVER_PORT 7878  
#define BUF_SIZE 4096 
#define QUEUE_SIZE 10
#define IPAddress "127.0.0.1" 

#endif

int main(void)
{
    WORD		wVersionRequested;
    WSADATA		wsaData;
    SOCKADDR_IN target;
    SOCKET      s;
    int			err;
    int			bytesSent;
    char        buf[50];

    wVersionRequested = MAKEWORD(1, 1);
    err = WSAStartup(wVersionRequested, &wsaData);

    if (err != 0) {
        printf("WSAStartup error %ld", WSAGetLastError());
        WSACleanup();
        return 0;
    }

    target.sin_family = AF_INET; 
    target.sin_port = htons(SERVER_PORT);
    target.sin_addr.s_addr = inet_addr(IPAddress); 


    s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (s == INVALID_SOCKET)
    {
        cout << "socket() error : "<< WSAGetLastError() << endl;
        WSACleanup();
        return 0;
    }


    if (connect(s, reinterpret_cast<SOCKADDR *>(&target), sizeof(target)) == SOCKET_ERROR)
    {
        cout << "connect() error : " << WSAGetLastError() << endl;
        cout << "서버 먼저 실행해주세요." << endl;
        WSACleanup();
        return 0; 
    }

    printf("Sending HELLO...\n");
    bytesSent = send(s, "HELLO", strlen("HELLO"), 0);

    int n;
    std::string sRand;
    int iRand;
    while (true)
    {
        try
        {
            n = recv(s, buf, 50, 0);
            if (n <= 0) { printf("Got nothing\n"); break; }
            buf[n] = 0;

            cout << "Received: " << buf << endl;
            sRand = buf;
            iRand = stoi(sRand);
            std::this_thread::sleep_for(std::chrono::seconds(1));
            cout << "Sending \"" << ++iRand << "\"" << " to client" << endl;
            sRand = std::to_string(iRand);
            bytesSent = send(s, sRand.c_str(), sRand.length(), 0);
        }
        catch (const std::invalid_argument &ex)
        {
            std::cerr << "Invalid argument while converting string to number" << endl;
            std::cerr << "Error: " << ex.what() << endl;
            break;
        }
        catch (const std::out_of_range &ex)
        {
            std::cerr << "Invalid argument while converting string to number" << endl;
            std::cerr << "Error: " << ex.what() << endl;
            break;
        }
    }

    closesocket(s);
    WSACleanup();

    return 0;

}