/*
 * Small wrapper on top of tauri api invoke
 *
 * We apply the idea of dependency injection to use a supplied invoke function as a
 * function argument, rather than directly referencing the Tauri invoke function.
 * Hence, don't import invoke globally in this file.
 */

import { invoke } from "@tauri-apps/api";

export async function ipc_invoke(
  method: string,
  params?: object
): Promise<any> {
  const response: any = await invoke(method, { params });
  if (response.error != null) {
    throw new Error(response.error);
  } else {
    return deepFreeze(response.result);
  }
}

function deepFreeze<T>(obj: T): T {
  if (Object.isFrozen(obj)) return obj;

  // Retrieve the property names defined on object
  const propNames = Object.getOwnPropertyNames(obj);

  // Freeze properties before freezing self

  for (const name of propNames) {
    const value = (<any>obj)[name];

    if (value != null && typeof value === "object") {
      deepFreeze(value);
    }
  }

  return Object.freeze(obj);
}
