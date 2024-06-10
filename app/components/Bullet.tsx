import { Route } from "@/types/Route";

export function Bullet(props: { route: Route; size: number }) {
  if (!props.route) {
    return;
  }

  const TEMP_TEXT_COLOR_OVERRIDE = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "5X",
    "6",
    "6X",
    "7",
    "7X",
    "B",
    "D",
    "F",
    "FX",
    "G",
    "H",
    "J",
    "L",
    "M",
    "S",
    "SIR",
    "T",
    "Z",
  ].includes(props.route.route_name);

  if (TEMP_TEXT_COLOR_OVERRIDE) {
    return (
      <span
        className="flex items-center justify-center"
        style={{
          height: `${props.size}px`,
          width: `${props.size}px`,
        }}
      >
        <span
          className="flex items-center justify-center rounded-full"
          style={{
            backgroundColor: `#${props.route.route_color}`,
            height: `${props.size}px`,
            width: `${props.size}px`,
          }}
        >
          <h1
            className="text-center font-bold"
            style={{
              fontSize: `${props.size * 0.65}px`,
              color: `#FFFFFF`,
            }}
          >
            {props.route.route_name}
          </h1>
        </span>
      </span>
    );
  }

  if (props.route.route_name.length <= 0) {
    return;
  }

  if (props.route.route_name.length <= 1) {
    return (
      <span
        className="flex items-center justify-center"
        style={{
          height: `${props.size}px`,
          width: `${props.size}px`,
        }}
      >
        <span
          className="flex items-center justify-center rounded-full"
          style={{
            backgroundColor: `#${props.route.route_color}`,
            height: `${props.size}px`,
            width: `${props.size}px`,
          }}
        >
          <h1
            className="text-center font-bold "
            style={{
              fontSize: `${props.size * 0.65}px`,
              color: `#${props.route.route_text_color}`,
            }}
          >
            {props.route.route_name}
          </h1>
        </span>
      </span>
    );
  }

  if (props.route.route_name.length <= 2 && props.route.route_name.substring(1) == "X") {
    return (
      <span
        className="flex items-center justify-center"
        style={{
          height: `${props.size}px`,
          width: `${props.size}px`,
        }}
      >
        <span
          className="flex rotate-[45deg] items-center justify-center"
          style={{
            backgroundColor: `#${props.route.route_color}`,
            height: `${props.size / Math.sqrt(2)}px`,
            width: `${props.size / Math.sqrt(2)}px`,
          }}
        >
          <h1
            className="rotate-[-45deg] text-nowrap text-center font-bold"
            style={{
              fontSize: `${props.size * 0.65}px`,
              color: `#${props.route.route_text_color}`,
            }}
          >
            {props.route.route_name.substring(0, 1)}
          </h1>
        </span>
      </span>
    );
  }

  if (props.route.route_name.length <= 3 && props.route.route_name == "SIR") {
    return (
      <span
        className="flex items-center justify-center"
        style={{
          height: `${props.size}px`,
          width: `${props.size}px`,
        }}
      >
        <span
          className="flex items-center justify-center rounded-full"
          style={{
            backgroundColor: `#${props.route.route_color}`,
            height: `${props.size}px`,
            width: `${props.size}px`,
          }}
        >
          <h1
            className="text-nowrap text-center font-bold"
            style={{
              fontSize: `${props.size * 0.5}px`,
              color: `#${props.route.route_text_color}`,
            }}
          >
            {props.route.route_name}
          </h1>
        </span>
      </span>
    );
  }

  return (
    <span
      className="flex items-center justify-center rounded-2xl"
      style={{
        backgroundColor: `#${props.route.route_color}`,
        height: `${props.size}px`,
      }}
    >
      <h1
        className="text-nowrap text-center font-bold"
        style={{
          fontSize: `${props.size * 0.65}px`,
          color: `#${props.route.route_text_color}`,
          paddingLeft: `${props.size * 0.175}px`,
          paddingRight: `${props.size * 0.175}px`,
        }}
      >
        {props.route.route_name}
      </h1>
    </span>
  );
}
