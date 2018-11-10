import { Component, OnInit } from "@angular/core";
import { SummaryModelService, Summary } from "../summary-model.service";
import { session } from "../model";

@Component({
  selector: "home-page",
  templateUrl: "./home-page.component.html",
  styleUrls: ["./home-page.component.css"]
})
export class HomePageComponent implements OnInit {
  summary?: Summary = null;

  constructor(private summaryModelService: SummaryModelService) {}

  ngOnInit() {
    this.summaryModelService.summary.subscribe(summary => {
      this.summary = summary;
    });

    if (session && session.currUser()) {
      this.summaryModelService.select();
    }
  }
}
