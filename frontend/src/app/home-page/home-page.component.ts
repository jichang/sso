import { Component, OnInit, OnDestroy } from "@angular/core";
import { SummaryModelService, Summary } from "../summary-model.service";
import { session } from "../model";
import { MatDialog, MatDialogRef } from "@angular/material/dialog";
import { Router } from "@angular/router";
import { Subscription } from "rxjs";

@Component({
  selector: "home-page",
  templateUrl: "./home-page.component.html",
  styleUrls: ["./home-page.component.css"]
})
export class HomePageComponent implements OnInit, OnDestroy {
  summary?: Summary = null;
  summarySubscription?: Subscription = null;

  constructor(
    private summaryModelService: SummaryModelService,
    public dialog: MatDialog,
    public router: Router
  ) {}

  ngOnInit() {
    this.summarySubscription = this.summaryModelService.summary.subscribe(
      summary => {
        this.summary = summary;
      }
    );

    if (session && session.currUser()) {
      this.summaryModelService.select();
    }
  }

  ngOnDestroy() {
    if (this.summarySubscription) {
      this.summarySubscription.unsubscribe();
      this.summarySubscription = null;
    }
  }
}
