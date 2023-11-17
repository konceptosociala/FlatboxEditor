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
    internal static extern void renderer_bind_camera(RendererHandle renderer, CameraHandle camera);
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
    internal static extern void scene_free(IntPtr scene);

    [DllImport(LibPath)]
    internal static extern CameraHandle camera_new();
    [DllImport(LibPath)]
    internal static extern void camera_free(IntPtr renderer);
}