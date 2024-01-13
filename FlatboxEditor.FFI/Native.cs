using System;
using System.Runtime.InteropServices;
using FlatboxEditor.Render;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.FFI;

internal class Native 
{
    public const string LibPath = "libnative";

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate IntPtr CallbackDelegate(string glFunctionName);

    [DllImport(LibPath)]
    internal static extern RendererHandle renderer_init(CallbackDelegate initGlFunction);
    [DllImport(LibPath)]
    internal static extern void renderer_render_scene(RendererHandle renderer, SceneHandle scene);
    [DllImport(LibPath)]
    internal static extern void renderer_render_grid(RendererHandle renderer, GridHandle grid);
    [DllImport(LibPath)]
    internal static extern void renderer_bind_camera(RendererHandle renderer, CameraHandle camera);
    [DllImport(LibPath)]
    internal static extern void renderer_bind_camera_grid(RendererHandle renderer, CameraHandle camera);
    [DllImport(LibPath)]
    internal static extern void renderer_clear(RendererHandle renderer, float r, float g, float b);
    [DllImport(LibPath)]
    internal static extern void renderer_free(IntPtr renderer);

    [DllImport(LibPath)]
    internal static extern SceneHandle scene_new();
    [DllImport(LibPath)]
    internal static extern SceneHandle scene_open(string path);
    [DllImport(LibPath)]
    internal static extern void scene_save(SceneHandle scene, string path);
    [DllImport(LibPath)]
    internal static extern void scene_add_model(SceneHandle scene, ModelHandle model);
    [DllImport(LibPath)]
    internal static extern void scene_free(IntPtr scene);

    [DllImport(LibPath)]
    internal static extern CameraHandle camera_new();
    [DllImport(LibPath)]
    internal static extern void camera_free(IntPtr renderer);

    [DllImport(LibPath)]
    internal static extern void logger_init();
    [DllImport(LibPath)]
    internal static extern void logger_info(string msg);
    [DllImport(LibPath)]
    internal static extern void logger_warn(string msg);
    [DllImport(LibPath)]
    internal static extern void logger_error(string msg);
    [DllImport(LibPath)]
    internal static extern void logger_debug(string msg);

    [DllImport(LibPath)]
    internal static extern ModelHandle model_cube();
    [DllImport(LibPath)]
    internal static extern void model_free(IntPtr model);

    [DllImport(LibPath)]
    internal static extern GridHandle grid_new(uint width, uint height, uint resolution, Color color);
    [DllImport(LibPath)]
    internal static extern void grid_free(IntPtr grid);
}