using Avalonia.Controls;
using Avalonia.Controls.Shapes;
using Avalonia.Layout;
using Avalonia.Markup.Declarative;
using Avalonia.Media;
using FlatboxEditor.Render;
using FlatboxEditor.UI.Components;
using FlatboxEditor.UI.Data;

using Color = Avalonia.Media.Color;
using Grid = Avalonia.Controls.Grid;

namespace FlatboxEditor.UI.View;

public class EditorView : ComponentBase
{
	[Inject] public EditorData? Model { get; set; }

    protected override object Build()
    {
        return new DockPanel().Children(
            new Menu()
                .Dock(Dock.Top)
                .Items(
                    new MenuItem()
                        .Header("File")
                        .Items(
                            new MenuItem().Header("Open..."),
                            new Separator(),
                            new MenuItem()
                                .Header("Exit")
                                
                        ),

					new MenuItem()
                        .Header("Edit")
                        .Items(
                            new MenuItem().Header("Copy"),
                            new MenuItem().Header("Paste")
                        )
                ),

            new TextBlock(),

            new Grid()
                .Cols("64, *, 0.1, 0.2*")
                .Children(
                    new StackPanel()
						.Col(0)
                        .Orientation(Orientation.Vertical)
                        .Children(
							// TODO: working buttons
                            new FlatButton("CursorRegular", (e) => {}),
							new FlatButton("CubeRegular", (e) => {}),
							new FlatButton("PaintBucketRegular", (e) => {}),
							new FlatButton("PersonRegular", (e) => {}),
							new FlatButton("CameraRegular", (e) => {})
                        ),

					new Grid()
						.Col(1)
						.Rows("48, *, 0.1, 0.3*")
						.Children(
							new StackPanel()
								.Row(0)
								.Classes("ButtonGroup")
								.Children(
									// TODO: Button group
								),

							new Editor3D()
								.Row(1),

							new GridSplitter()
								.Row(2)
								.ResizeDirection(GridResizeDirection.Rows)
								.Background(new SolidColorBrush(App.GetResource<Color>("SystemRegionColor")!)),

							new TabControl()
								.Row(3)
								.Items(
									new TabItem()
										.Header("Asset Manager")
										.VerticalContentAlignment(VerticalAlignment.Center)
										.Content(
											new TextBlock()
												.Text("<Asset Manager Tab>")
												.HorizontalAlignment(HorizontalAlignment.Center)
												.VerticalAlignment(VerticalAlignment.Center)
										),

									new TabItem()
										.Header("Material")
										.VerticalContentAlignment(VerticalAlignment.Center)
										.Content(
											new TextBlock()
												.Text("<Material Tab>")
												.HorizontalAlignment(HorizontalAlignment.Center)
												.VerticalAlignment(VerticalAlignment.Center)
										),

									new TabItem()
										.Header("Animator")
										.VerticalContentAlignment(VerticalAlignment.Center)
										.Content(
											new TextBlock()
												.Text("<Animator Tab>")
												.HorizontalAlignment(HorizontalAlignment.Center)
												.VerticalAlignment(VerticalAlignment.Center)
										)
								)
						),

					new GridSplitter()
						.Col(2)
						.ResizeDirection(GridResizeDirection.Columns)
						.Background(new SolidColorBrush(App.GetResource<Color>("SystemRegionColor")!)),

					new Grid()
						.Col(3)
						.Rows("*, 0.1, *")
						.Children(
							new Rectangle()
								.Row(0)
								.Fill(new SolidColorBrush(App.GetResource<Color>("SystemRegionColor")!)),
								
							new GridSplitter()
								.Row(1)
								.ResizeDirection(GridResizeDirection.Rows)
								.Background(new SolidColorBrush(App.GetResource<Color>("SystemRegionColor")!)),

							new Rectangle()
								.Row(2)
								.Fill(new SolidColorBrush(App.GetResource<Color>("SystemRegionColor")!))
						)
                )
        );
    }

    protected override void OnSizeChanged(SizeChangedEventArgs e)
	{
		StateHasChanged();
		base.OnSizeChanged(e);
	}
}