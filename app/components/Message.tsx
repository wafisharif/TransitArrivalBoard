import { Route } from "@/types/Route";
import React from "react";
import { Bullet } from "./Bullet";

export function Message(props: { routes: Record<string, Route>; name: string; headers: Array<string>; index: number }) {
  return (
    <React.Fragment>
      <div className="flex min-h-14 flex-row items-center rounded-lg bg-red-600">
        {props.headers.length > 0 ? (
          <h1 className="mx-2 font-black text-white 2xl:text-3xl">
            {props.name + " (" + (props.index + 1) + "/" + props.headers.length + ")"}
          </h1>
        ) : (
          <h1 className="mx-2 font-black text-white 2xl:text-3xl">{props.name}</h1>
        )}
      </div>
      {props.headers.length > 0 ? (
        <div className="flex min-h-62 flex-row items-start rounded-lg bg-slate-100 px-4">
          <h1 className="line-clamp-4 text-pretty font-semibold 2xl:text-5xl 2xl:leading-tight">
            {props.headers[props.index].split(/(\[.*?\])/).map((text, index) => {
              if (text.length == 0) return;
              if (text.substring(0, 1) != "[" || text.substring(text.length - 1) != "]") return text;
              return (
                <div className="mx-1 inline-flex -translate-y-1.5" key={index}>
                  <Bullet route={props.routes[text.substring(1, text.length - 1)]} size={42} />
                </div>
              );
            })}
          </h1>
        </div>
      ) : (
        <div className="flex min-h-62 flex-row items-center rounded-lg bg-slate-100">
          <h1 className="flex-1 text-center font-bold text-black 2xl:text-5xl">No active alerts</h1>
        </div>
      )}
    </React.Fragment>
  );
}
