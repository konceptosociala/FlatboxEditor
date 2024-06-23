using System;
using System.Runtime.InteropServices;
using Avalonia.OpenGL;
using FlatboxEditor.FFI;
using FlatboxEditor.Render;

namespace FlatboxEditor.Scenes;

internal class SceneHandle : SafeHandle 
{
    public SceneHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) {
            Native.scene_free(handle);
        }

        return true;
    }
}

public class Scene : IDisposable
{
    internal readonly SceneHandle scene;

    public Scene()
    {
        scene = Native.scene_new();
    }

    public Scene(string path)
    {
        scene = Native.scene_open(path);
    }

    public void AddModel(Model model) {
        Native.scene_add_model(scene, model._model);
    }

    public void Save(string path)
    {
        Native.scene_save(scene, path);
    }

    public void Dispose()
    {
        scene.Dispose();
    }
}