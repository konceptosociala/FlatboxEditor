using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

public class Logger
{
    public Logger()
    {
        Native.logger_init();
    }

    public void Info(string msg) {
        Native.logger_info(msg);
    }

    public void Warn(string msg) {
        Native.logger_warn(msg);
    }

    public void Error(string msg) {
        Native.logger_error(msg);
    }

    public void Debug(string msg) {
        Native.logger_debug(msg);
    }
}