import Reqwest from '@kingtous/reqwest';

@Entry
@Component
struct Index {
  @State url: string = 'https://192.168.0.100';
  @State ignore_ssl: boolean = false;

  triggerRequest() {
    Reqwest.safeRequest(this.url, {}, "GET", "", this.ignore_ssl).then((resp) => {
      AlertDialog.show({
        title: "请求结果",
        message: `${resp}`
      });
    }).catch((error) => {
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
            left:4
          })
        }.justifyContent(FlexAlign.Center)
        Divider()
        Button() {
          Text("请求").fontColor(Color.White).margin(8)
        }.onClick((_) => {
          this.triggerRequest();
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}