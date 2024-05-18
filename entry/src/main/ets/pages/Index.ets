import reqwest, { ReqwestCertType } from '@kingtous/reqwest';
import { AbortController } from '@ohos-rs/abort-controller';

@Entry
@Component
struct Index {
  @State url: string = 'https://192.168.0.101:2088';
  @State ignore_ssl: boolean = false;
  @State noProxy: boolean = false;
  @State caCert: string = `MIIBtjCCAVugAwIBAgITBmyf1XSXNmY/Owua2eiedgPySjAKBggqhkjOPQQDAjA5
MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6b24g
Um9vdCBDQSAzMB4XDTE1MDUyNjAwMDAwMFoXDTQwMDUyNjAwMDAwMFowOTELMAkG
A1UEBhMCVVMxDzANBgNVBAoTBkFtYXpvbjEZMBcGA1UEAxMQQW1hem9uIFJvb3Qg
Q0EgMzBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABCmXp8ZBf8ANm+gBG1bG8lKl
ui2yEujSLtf6ycXYqm0fc4E7O5hrOXwzpcVOho6AF2hiRVd9RFgdszflZwjrZt6j
QjBAMA8GA1UdEwEB/wQFMAMBAf8wDgYDVR0PAQH/BAQDAgGGMB0GA1UdDgQWBBSr
ttvXBp43rDCGB5Fwx5zEGbF4wDAKBggqhkjOPQQDAgNJADBGAiEA4IWSoxe3jfkr
BqWTrBqYaGFy+uGh0PsceGCmQ5nFuMQCIQCcAu/xlJyzlvnrxir4tiz+OpAUFteM
YyRIHN8wfdVoOw==`
  @State timeout: number = 5000;
  @State forceRustls: boolean = true;
  // abort
  private abortController: AbortController = new AbortController();


  triggerRequest() {
    reqwest.request(this.url, "GET", {
      ignoreSsl: this.ignore_ssl,
      noProxy: this.noProxy,
      responseType: "application/json",
      caCert: [
        {
          ty: ReqwestCertType.PEM,
          cert: this.caCert
        }
      ],
      timeout: this.timeout
    }, {
      signal: this.abortController.signal
    }).then((resp) => {
      AlertDialog.show({
        title: "请求结果",
        message: `${resp}`
      });
    }).catch((error: Error) => {
      AlertDialog.show({
        title: "请求失败",
        message: `${error}`
      });
    });
  }

  build() {
    Row() {
      Column() {
        TextInput({
          text: this.url,
        }) .onChange((value)=> {
          this.url = value;
        })
        Row() {
          Checkbox({
            group: '完全忽略SSL证书',
            name: "完全忽略SSL证书"
          }).select(this.ignore_ssl).onChange((c) => {
            this.ignore_ssl = c;
          })
          Text("完全忽略SSL证书").margin({
            left:4,
            right:4
          })

          Checkbox({
            group: '默认使用rustls',
            name: "默认使用rustls"
          }).select(this.forceRustls).onChange((c) => {
            this.forceRustls = c;
          })
          Text("默认使用rustls").margin({
            left:4
          })
        }.justifyContent(FlexAlign.Center)
        Row(){
          Text("CA证书")
          TextInput({
            text: this.caCert
          }).onChange((ca) => {
            this.caCert = ca;
          }).width(300)
        }
        Divider()
        Button() {
          Text("请求").fontColor(Color.White).margin(8)
        }.onClick((_) => {
          this.triggerRequest();
        })
        Button() {
          Text("取消请求").fontColor(Color.White).margin(8)
        }.onClick((_) => {
          this.abortController.abort('被取消');
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}