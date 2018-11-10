import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";

@Component({
  selector: "application-create-page",
  templateUrl: "./application-create-page.component.html",
  styleUrls: ["./application-create-page.component.css"]
})
export class ApplicationCreatePageComponent implements OnInit {
  constructor(private router: Router, private route: ActivatedRoute) {}

  ngOnInit() {}

  created() {
    this.router.navigate(["../"], { relativeTo: this.route });
  }
}
