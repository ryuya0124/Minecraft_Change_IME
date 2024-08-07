#include <windows.h>
#include <shellapi.h>
#include <stdio.h>

#define TARGET_WINDOW_TITLE "Minecraft"
#define CHECK_INTERVAL 100
#define WM_TRAYICON (WM_USER + 1)
#define ID_TRAY_EXIT 1001

NOTIFYICONDATA nid;
BOOL wasActive = FALSE;
HANDLE hMutex;
HANDLE hThread; // スレッドのハンドルを追加

void sendKeyCombination(WORD key1, WORD key2, WORD key3) {
    INPUT inputs[6] = {0};

    inputs[0].type = INPUT_KEYBOARD;
    inputs[0].ki.wVk = key1;

    inputs[1].type = INPUT_KEYBOARD;
    inputs[1].ki.wVk = key2;

    inputs[2].type = INPUT_KEYBOARD;
    inputs[2].ki.wVk = key3;

    inputs[3].type = INPUT_KEYBOARD;
    inputs[3].ki.wVk = key3;
    inputs[3].ki.dwFlags = KEYEVENTF_KEYUP;

    inputs[4].type = INPUT_KEYBOARD;
    inputs[4].ki.wVk = key2;
    inputs[4].ki.dwFlags = KEYEVENTF_KEYUP;

    inputs[5].type = INPUT_KEYBOARD;
    inputs[5].ki.wVk = key1;
    inputs[5].ki.dwFlags = KEYEVENTF_KEYUP;

    SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));
}

void createTrayIcon(HWND hwnd) {
    memset(&nid, 0, sizeof(nid));
    nid.cbSize = sizeof(nid);
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = LoadIcon(NULL, IDI_APPLICATION);
    strcpy(nid.szTip, "Minecraft_Change_IME");

    Shell_NotifyIcon(NIM_ADD, &nid);
}

void removeTrayIcon() {
    Shell_NotifyIcon(NIM_DELETE, &nid);
}

void showContextMenu(HWND hwnd) {
    POINT pt;
    HMENU hMenu = CreatePopupMenu();

    AppendMenu(hMenu, MF_STRING, ID_TRAY_EXIT, TEXT("Exit"));
    
    GetCursorPos(&pt);
    SetForegroundWindow(hwnd);
    TrackPopupMenu(hMenu, TPM_BOTTOMALIGN | TPM_LEFTALIGN, pt.x, pt.y, 0, hwnd, NULL);
    DestroyMenu(hMenu);
}

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch (msg) {
        case WM_TRAYICON:
            if (lParam == WM_RBUTTONUP) {
                showContextMenu(hwnd);
            }
            break;
        case WM_COMMAND:
            if (LOWORD(wParam) == ID_TRAY_EXIT) {
                PostQuitMessage(0);
            }
            break;
        case WM_DESTROY:
            PostQuitMessage(0);
            break;
    }
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

DWORD WINAPI monitorThread(LPVOID lpParam) {
    HWND hwnd;
    char windowTitle[256];

    while (1) {
        hwnd = GetForegroundWindow();
        if (hwnd) {
            GetWindowTextA(hwnd, windowTitle, sizeof(windowTitle));

            if (strcmp(windowTitle, TARGET_WINDOW_TITLE) == 0) {
                if (!wasActive) {
                    sendKeyCombination(VK_MENU, VK_SHIFT, '2'); // Alt + Shift + 2
                    wasActive = TRUE;
                }
            } else {
                if (wasActive) {
                    sendKeyCombination(VK_MENU, VK_SHIFT, '1'); // Alt + Shift + 1
                    wasActive = FALSE;
                }
            }
        }
        Sleep(CHECK_INTERVAL);
    }

    return 0;
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    WNDCLASS wc = {0};
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = TEXT("Minecraft_Change_IMEClass");

    // ミューテックスを作成して、プログラムが既に実行中かどうかを確認
    hMutex = CreateMutex(NULL, TRUE, TEXT("MyUniqueMutexName"));
    if (GetLastError() == ERROR_ALREADY_EXISTS) {
        // 既に実行中のインスタンスがある場合は終了
        MessageBox(NULL, TEXT("Program is already running."), TEXT("Info"), MB_OK | MB_ICONINFORMATION);
        return 0;
    }

    if (!RegisterClass(&wc)) {
        return 1;
    }

    HWND hwnd = CreateWindow(wc.lpszClassName, TEXT("Minecraft_Change_IME"), 0, 0, 0, 0, 0, NULL, NULL, hInstance, NULL);

    if (!hwnd) {
        return 1;
    }

    createTrayIcon(hwnd);

    hThread = CreateThread(NULL, 0, monitorThread, NULL, 0, NULL);
    if (!hThread) {
        MessageBox(NULL, TEXT("Failed to create monitoring thread."), TEXT("Error"), MB_OK | MB_ICONERROR);
        return 1;
    }

    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    // プログラム終了時にスレッドを終了
    TerminateThread(hThread, 0);
    CloseHandle(hThread);

    // プログラム終了時にミューテックスを解放
    ReleaseMutex(hMutex);
    CloseHandle(hMutex);

    removeTrayIcon();

    return 0;
}
