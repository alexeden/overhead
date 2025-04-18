// @ts-nocheck
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/

export const commands = {
  async discover(): Promise<Result<Device[], AppError>> {
    try {
      return { status: 'ok', data: await TAURI_INVOKE('discover') };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: 'error', error: e as any };
    }
  },
  async setBrightness(
    socketAddr: string,
    brightness: number
  ): Promise<Result<null, AppError>> {
    try {
      return {
        status: 'ok',
        data: await TAURI_INVOKE('set_brightness', { socketAddr, brightness }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: 'error', error: e as any };
    }
  },
  async toggle(socketAddr: string): Promise<Result<boolean, AppError>> {
    try {
      return {
        status: 'ok',
        data: await TAURI_INVOKE('toggle', { socketAddr }),
      };
    } catch (e) {
      if (e instanceof Error) throw e;
      else return { status: 'error', error: e as any };
    }
  },
};

/** user-defined events **/

/** user-defined constants **/

/** user-defined types **/

export type AppError =
  /**
   * Tried to send a command to a device that was never discovered
   */
  | { NotFound: string }
  /**
   * TPLinker error
   */
  | { Tp: TpError };
export type Device = {
  addr: string;
  id: string;
  model: string;
  name: string;
  brightness: number | null;
  isOn: boolean;
};
/**
 * Error response for a section of the JSON response
 */
export type SectionError = {
  /**
   * The error code. Zero if no error.
   */
  err_code: number | null;
  /**
   * The error message.
   */
  err_msg: string | null;
};
/**
 * Error type for TPLinker
 */
export type TpError =
  /**
   * Wrapped errors from std::io
   */
  | { IO: string }
  /**
   * Wrapped errors from serde_json
   */
  | { Serde: string }
  /**
   * Error decoding a section of the JSON response
   */
  | { TPLink: SectionError }
  /**
   * Unknown device model
   */
  | { UnknownModel: string }
  /**
   * Tried to use a feature that is not supported by the device
   */
  | { Unsupported: string }
  /**
   * A generic error
   */
  | { Unknown: string };

/** tauri-specta globals **/

import {
  invoke as TAURI_INVOKE,
  Channel as TAURI_CHANNEL,
} from '@tauri-apps/api/core';
import * as TAURI_API_EVENT from '@tauri-apps/api/event';
import { type WebviewWindow as __WebviewWindow__ } from '@tauri-apps/api/webviewWindow';

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: null extends T
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
  | { status: 'ok'; data: T }
  | { status: 'error'; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindow__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindow__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case 'listen':
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case 'once':
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case 'emit':
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}
