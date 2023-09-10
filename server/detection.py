from ultralytics import YOLO
import cv2

model = YOLO("best.pt")
names_ru = {
    2: "Продольная трещина",
    4: "Поперечная трещина",
    0: "Аллегаторная трещина",
    1: "Колея, неровность, выбоина, расслоение",
    5: "Размытие пешеходного перехода",
    3: "Размытие дорожной разметки",
    7: "Ремонт",
    6: "Служебное отверстие (люк для обслуживания)",
}
names_eng = {
    2: "Linear longitudinal crack",
    4: "Linear lateral crack",
    0: "Alligator crack",
    1: "Rutting, bump, pothole, separation",
    5: "Cross walk blur",
    3: "White line blur",
    7: "Repair",
    6: "Utility hole (maintenance hatch)",
}
model.names.update(names_ru)


def predict(input_arr, output_path):
    pred = model.predict(input_arr, iou=0.3, conf=0.1, imgsz=(640, 640), augment=True)
    pred_labels = [names_ru[x] for x in pred[0].boxes.cls.int().tolist()]
    pred_boxes = pred[0].boxes.xyxy.int().tolist()
    result = cv2.cvtColor(pred[0].plot(), cv2.COLOR_BGR2RGB)
    cv2.imwrite(output_path, result)
    return zip(pred_labels, pred_boxes)
