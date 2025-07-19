import { useAppProps } from "antd/es/app/context";

export function success({ notification }: useAppProps, message: React.ReactNode, description: string) {
  notification.success({
    message,
    description,
    placement: "bottom",
  });
}

export function info({ notification }: useAppProps, message: React.ReactNode, description: string) {
  notification.info({
    message,
    description,
    placement: "bottom",
  });
}

export function warning({ notification }: useAppProps, message: React.ReactNode, description: string) {
  notification.warning({
    message,
    description,
    placement: "bottom",
  });
}

export function error({ notification }: useAppProps, message: React.ReactNode, description: string) {
  notification.error({
    message,
    description,
    placement: "bottom",
  });
}
