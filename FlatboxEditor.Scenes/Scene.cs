using System.Runtime.InteropServices;
using FlatboxEditor.FFI;
using FlatboxEditor.Render;

namespace FlatboxEditor.Scenes;

internal class SceneHandle : SafeHandle 
{
    public SceneHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid 
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (!IsInvalid) 
        {
            NativeInterface.scene_free(handle);
        }

        return true;
    }
}

public class Scene : IDisposable
{
    internal readonly SceneHandle _scene;

    public Scene()
    {
        _scene = NativeInterface.scene_new();
    }

    public Scene(string path)
    {
        _scene = NativeInterface.scene_open(path);
    }

    public void AddModel(Model model) {
        NativeInterface.scene_add_model(_scene, model.Native());
    }

    public void Save(string path)
    {
        NativeInterface.scene_save(_scene, path);
    }

    public void Dispose()
    {
        _scene.Dispose();
    }

    internal SceneHandle Native() => _scene;
}