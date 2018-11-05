import { Component, OnInit, Input } from "@angular/core";
import { SummaryModelService, Summary } from "../summary-model.service";

@Component({
  selector: "summary-panel",
  templateUrl: "./summary-panel.component.html",
  styleUrls: ["./summary-panel.component.css"]
})
export class SummaryPanelComponent implements OnInit {
  @Input()
  summary: Summary;

  constructor() {}

  ngOnInit() {}
}
