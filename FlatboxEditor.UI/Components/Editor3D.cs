using Avalonia.OpenGL;
using Avalonia.OpenGL.Controls;
using Avalonia.Threading;
using FlatboxEditor.Render;
using FlatboxEditor.Scenes;

namespace FlatboxEditor.UI.Components;

public class Editor3D : OpenGlControlBase
{
    private bool init = false;

    private Renderer? _renderer;
    private Camera? _camera;
    private Scene? _scene;
    private Grid? _grid;

    protected override void OnOpenGlInit(GlInterface gl)
    {
        if (init) return;

        base.OnOpenGlInit(gl);

        _renderer = new Renderer(gl);
        _camera = new Camera();
        _scene = new Scene();
        _grid = new Grid(20, 20, 10, Color.Orange());
        _scene.AddModel(Model.Cube());

        init = true;
    }

    protected override void OnOpenGlDeinit(GlInterface gl)
    {
        base.OnOpenGlDeinit(gl);
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        gl.Viewport(0,0, (int)Bounds.Width, (int)Bounds.Height);
        _renderer!.Clear(0.5f, 0.5f, 1.0f);
        _renderer!.BindCamera(_camera!);
        _renderer!.BindCameraGrid(_camera!);
        _renderer!.RenderScene(_scene!);
        _renderer!.RenderGrid(_grid!);

        Dispatcher.UIThread.Post(RequestNextFrameRendering, DispatcherPriority.Background);
    }
}
