/* tslint:disable */
/* eslint-disable */

/**
 * The `MediaSourceEnum` enum.
 *
 * *This API requires the following crate features to be activated: `MediaSourceEnum`*
 */
export enum MediaSourceEnum {
    Camera = "camera",
    Screen = "screen",
    Application = "application",
    Window = "window",
    Browser = "browser",
    Microphone = "microphone",
    AudioCapture = "audioCapture",
    Other = "other",
}

export function get_media_source(): MediaSourceEnum;

export function get_url(): URL;
