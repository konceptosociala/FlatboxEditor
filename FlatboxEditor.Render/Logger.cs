using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

public static class Logger
{
    static Logger()
    {
        Native.logger_init();
    }

    public static void Info(string msg) {
        Native.logger_info(msg);
    }

    public static void Warn(string msg) {
        Native.logger_warn(msg);
    }

    public static void Error(string msg) {
        Native.logger_error(msg);
    }

    public static void Debug(string msg) {
        Native.logger_debug(msg);
    }
}