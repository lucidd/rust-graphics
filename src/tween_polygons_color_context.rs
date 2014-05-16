use {
    BackEnd,
    Borrowed,
    Clear,
    Color,
    Field,
    Fill,
    Matrix2d,
    Value,
    View,
};
use vecmath::{
    identity
};
use triangulation::{
    with_lerp_polygons_tri_list_xy_f32_rgba_f32
};
use internal::{
    CanColor,
    CanTransform,
    HasColor,
    HasTransform,
};

/// An animation inbetweening context with color.
pub struct TweenPolygonsColorContext<'a, 'b> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
    /// The animated polygons.
    pub polygons: Field<'a, &'b [&'b [f64]]>,
}

impl<'a, 'b> Clone for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn clone(&self) -> TweenPolygonsColorContext<'static, 'b> {
        TweenPolygonsColorContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            color: self.color.clone(),
            tween_factor: self.tween_factor.clone(),
            polygons: self.polygons.clone(),
        }
    }
}

impl<'a, 'b> HasColor<'a, Color> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a, 'b> CanColor<'a, TweenPolygonsColorContext<'a, 'b>, Color> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> HasTransform<'a, Matrix2d> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(alwyas)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a, 'b> CanTransform<'a, TweenPolygonsColorContext<'a, 'b>, Matrix2d> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
            polygons: Borrowed(self.polygons.get()),
        }
    }
}

impl<'a, 'b> Fill<'a> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn fill<B: BackEnd>(&'a self, back_end: &mut B) {
        if back_end.supports_tri_list_xy_f32_rgba_f32() {
            let polygons = self.polygons.get();
            let &Color(color) = self.color.get();
            // Complete transparency does not need to be rendered.
            if color[3] == 0.0 { return; }
            // Turn on alpha blending if not completely opaque.
            let needs_alpha = color[3] != 1.0;
            if needs_alpha { back_end.enable_alpha_blend(); }
            with_lerp_polygons_tri_list_xy_f32_rgba_f32(
                self.transform.get(),
                *polygons,
                *self.tween_factor.get(),
                &Color(color),
                |vertices, colors| {
                    back_end.tri_list_xy_f32_rgba_f32(vertices, colors)
                }
            );
            if needs_alpha { back_end.disable_alpha_blend(); }
        } else {
            unimplemented!();
        }
    }
}

impl<'a, 'b> View<'a> for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn view(&'a self) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            polygons: Borrowed(self.polygons.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            polygons: Borrowed(self.polygons.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> TweenPolygonsColorContext<'a, 'b> {
        TweenPolygonsColorContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            polygons: Borrowed(self.polygons.get()),
            color: Borrowed(self.color.get()),
            tween_factor: Borrowed(self.tween_factor.get()),
        }
    }
}

impl<'a, 'b> Clear for TweenPolygonsColorContext<'a, 'b> {
    #[inline(always)]
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let &Color(color) = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

