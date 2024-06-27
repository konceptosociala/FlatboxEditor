using FlatboxEditor.FFI;

namespace FlatboxEditor.Render;

public static class Logger
{
    static Logger()
    {
        NativeInterface.logger_init();
    }

    public static void Info(string msg) {
        NativeInterface.logger_info(msg);
    }

    public static void Warn(string msg) {
        NativeInterface.logger_warn(msg);
    }

    public static void Error(string msg) {
        NativeInterface.logger_error(msg);
    }

    public static void Debug(string msg) {
        NativeInterface.logger_debug(msg);
    }
}