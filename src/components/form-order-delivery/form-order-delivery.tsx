// import { ToggleCarDelivery } from "@/components";
import { TimeIcon } from "@/components/delivery/icons/time-icon.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { Input } from "@/components/ui/input.tsx";
import { cn } from "@/lib/utils.ts";
import { ReceiveIcon } from "@/components/delivery/icons/receive-icon.tsx";
import { SendIcon } from "@/components/delivery/icons/send-icon.tsx";
import { Button } from "@/components/ui/button.tsx";
import { RightIcon } from "@/components/delivery/icons/right-icon.tsx";
import { YesIcon } from "@/components/delivery/icons/yes-icon.tsx";
import { z } from "zod";
import { type ControllerRenderProps, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
} from "@/components/ui/form.tsx";
import { useEffect, useState } from "react";
import MapPicker from "@/components/delivery/map-picker/map-picker.tsx";
import { Badge } from "@/components/ui/badge";
import { LoadingSpinner } from "@/components/form-order-delivery/loading-spinner.tsx";
import { FormSchemaForCreateDeliveryTask } from "@/api";
import { useTaskCreate } from "@/hooks/query/use-task-create.ts";
// import { FormSchemaForCreateDeliveryTask } from "@/api/task-delivery-create.ts";

interface MapTrigger {
  display: boolean;
  triggerCallback?: any;
}

export function FormOrderDelivery() {
  const form = useForm<z.infer<typeof FormSchemaForCreateDeliveryTask>>({
    resolver: zodResolver(FormSchemaForCreateDeliveryTask),
    defaultValues: {
      shipping_method: "motor",
      address_starting: {},
      address_receive: {},
      floor_room_number: "",
      name_receive: "",
      phone_number_receive: "",
      need_save: true,
    },
  });

  // const queryTaskForCreate = useQueryTask(Query_create_delivery_task)

  const createMutation = useTaskCreate();

  const onSubmit = (data: z.infer<typeof FormSchemaForCreateDeliveryTask>) => {
    createMutation.mutate(data);
    // const {isLoading, data: result} = queryTaskForCreate(data)
    // console.log("submit", isLoading, result);
    // console.log(data);
  };

  const displayMapIframeForChooseAddress = (
    field: ControllerRenderProps<
      z.infer<typeof FormSchemaForCreateDeliveryTask>
    >,
  ) => {
    setMapTrigger({
      display: true,
      triggerCallback: (location: any) => {
        field.onChange(location);
        setSelectedLocation(location);
      },
    });
  };

  useEffect(() => {
    console.log(form.getValues());
  }, [form.formState]);

  const [mapTrigger, setMapTrigger] = useState<MapTrigger>({
    display: false,
    triggerCallback: undefined,
  });
  const [selectedLocation, setSelectedLocation] = useState<any>(null);

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className="w-full pt-2 bg-gray-50 rounded-2xl shadow-[0px_-0.5px_4px_0px_rgba(0,0,0,0.05)] inline-flex flex-col justify-start items-center gap-1 overflow-hidden"
      >
        {selectedLocation && (
          <div className="mt-4 p-4 border rounded">
            <h2 className="text-xl font-semibold">已选择的位置</h2>
            <p>名称: {selectedLocation.poiname}</p>
            <p>地址: {selectedLocation.poiaddress}</p>
            <p>
              经纬度: {selectedLocation.latlng.lat},{" "}
              {selectedLocation.latlng.lng}
            </p>
            <p>城市: {selectedLocation.cityname}</p>
          </div>
        )}
        <MapPicker
          visible={mapTrigger.display}
          onLocationSelect={mapTrigger.triggerCallback}
          onClose={() => setMapTrigger({ display: false })}
          apiKey="44OBZ-NDTOT-LRJXQ-VQ7TW-2YTZT-K4FR5"
          referer="paotui"
        />

        {/* Header */}
        <div className="w-full h-[90px] relative">
          <div className="w-full px-1.5 py-px left-0 top-0 absolute inline-flex justify-between items-center overflow-hidden">
            {/*<ToggleCarDelivery />*/}
            <div className="w-[120px] flex justify-start items-center gap-[3px] overflow-hidden">
              <TimeIcon />
              <div className="w-[99px] h-[15px] justify-center text-gray-500 text-[12px] font-normal ">
                预计1分钟内接单
              </div>
            </div>
          </div>

          <div className="w-full h-12 px-1.5 left-0 top-[42px] absolute inline-flex justify-start items-center gap-3 overflow-hidden">
            <SendIcon />
            <div className="w-full justify-center text-black text-[16px] font-semibold">
              五谷粥杂粮(对外经贸学院店)
            </div>

            <FormField
              control={form.control}
              name="address_starting"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Button
                      onClick={() => displayMapIframeForChooseAddress(field)}
                      variant={"link"}
                      className={
                        "has-[>svg]:px-1 px-0 gap-0 flex font-semibold cursor-pointer"
                      }
                    >
                      更换地址
                      <RightIcon color={"black"} />
                    </Button>
                  </FormControl>
                </FormItem>
              )}
            />
          </div>
        </div>

        <div className="self-stretch h-[296.10px] py-1 bg-white rounded-2xl shadow-[0.5px_0.5px_2px_0px_rgba(0,0,0,0.10)] flex flex-col justify-start items-center">
          <FormField
            control={form.control}
            name="address_receive"
            render={({ field }) => (
              <FormItem className={"w-full"}>
                <FormControl className={"w-full"}>
                  <div className="w-full h-12 px-1.5 inline-flex justify-start items-center gap-3 overflow-hidden">
                    <ReceiveIcon />
                    <div
                      onClick={() => displayMapIframeForChooseAddress(field)}
                      className="flex cursor-pointer w-full h-5 justify-between items-center text-slate-900 text-[16px] font-semibold "
                    >
                      选择收货地址(必填)
                      <RightIcon color={"black"} />
                    </div>
                    <Badge
                      variant="secondary"
                      className={
                        "mr-2 cursor-pointer border-none bg-[#F0F3F8] rounded-3xl"
                      }
                    >
                      地址簿
                    </Badge>
                  </div>
                </FormControl>
              </FormItem>
            )}
          />
          <Separator className={"opacity-50"} />
          <FormField
            control={form.control}
            name="floor_room_number"
            render={({ field }) => (
              <FormItem className={"w-full"}>
                <FormControl className={"w-full"}>
                  <Input
                    {...field}
                    placeholder={"楼层或门牌号"}
                    className={cn(
                      "placeholder:text-[#C7C7C7] h-[48px] px-1.5 inline-flex justify-start items-center overflow-hidden border-none",
                    )}
                  />
                </FormControl>
              </FormItem>
            )}
          />

          <Separator className={"opacity-50"} />
          <FormField
            control={form.control}
            name="name_receive"
            render={({ field }) => (
              <FormItem className={"w-full"}>
                <FormControl className={"w-full"}>
                  <Input
                    {...field}
                    placeholder={"姓名"}
                    className={cn(
                      "placeholder:text-[#C7C7C7] h-[48px] px-1.5 inline-flex justify-start items-center overflow-hidden border-none",
                    )}
                  />
                </FormControl>
              </FormItem>
            )}
          />

          <Separator className={"opacity-50"} />
          <FormField
            control={form.control}
            name="phone_number_receive"
            render={({ field }) => (
              <FormItem className={"w-full"}>
                <FormControl className={"w-full"}>
                  <Input
                    {...field}
                    placeholder={"手机(必填)"}
                    className={cn(
                      "placeholder:text-[#C7C7C7] h-[48px] px-1.5 inline-flex justify-start items-center overflow-hidden border-none",
                    )}
                  />
                </FormControl>
              </FormItem>
            )}
          />

          <Separator className={"opacity-50"} />

          <div className="w-full flex gap-2 h-24 items-center ">
            <YesIcon className={"pl-1"} />
            <span className="w-[82px] justify-center text-slate-900 text-base font-medium ">
              存入地址簿
            </span>
          </div>

          {/* Footer */}
          <div className="self-stretch px-4 flex flex-col justify-center items-center">
            <Button
              type={"submit"}
              variant={"link"}
              disabled={!form.formState.isValid || form.formState.isSubmitting}
              className={cn(
                form.formState.isValid ? "bg-blue-500" : "bg-neutral-300",
                "self-stretch h-11 relative rounded-3xl overflow-hidden text-white cursor-pointer",
              )}
            >
              {form.formState.isSubmitting ? <LoadingSpinner /> : "立即下单"}
            </Button>
          </div>
        </div>
      </form>
    </Form>
  );
}
